#!/bin/bash

# Default values
CLICKHOUSE_USER="default"
CLICKHOUSE_PASSWORD="password"
CLICKHOUSE_HOST="localhost"
CLICKHOUSE_PORT="9000"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  key="$1"
  case $key in
    --user)
      CLICKHOUSE_USER="$2"
      shift
      shift
      ;;
    --password)
      CLICKHOUSE_PASSWORD="$2"
      shift
      shift
      ;;
    --host)
      CLICKHOUSE_HOST="$2"
      shift
      shift
      ;;
    --port)
      CLICKHOUSE_PORT="$2"
      shift
      shift
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Check if ClickHouse client is installed
if ! command -v clickhouse-client &> /dev/null; then
  echo "Error: clickhouse-client not found."
  echo "Please install ClickHouse client first."
  exit 1
fi

# Connection string
CLICKHOUSE_CONN="--user=$CLICKHOUSE_USER --password=$CLICKHOUSE_PASSWORD --host=$CLICKHOUSE_HOST --port=$CLICKHOUSE_PORT"

# Create a dummy query to test the connection
echo "Testing connection to ClickHouse server..."
if ! clickhouse-client $CLICKHOUSE_CONN --query="SELECT 1;" &> /dev/null; then
  echo "Error: Could not connect to ClickHouse server."
  echo "Please check your credentials and server status."
  exit 1
fi
echo "Connection successful!"

# Extract the schema from the clickhouse-sink.yaml file
echo "Extracting schema from clickhouse-sink.yaml..."

# Extract schema definitions using pattern matching
SCHEMA_FILE="clickhouse-schema.sql"

# Clean up the schema file if it exists
> $SCHEMA_FILE

# Parse the clickhouse-sink.yaml to extract the schema
cat clickhouse-sink.yaml | awk '/schema:/,/maps:/' | grep -E '  - name:|    engine:|    primary_key:|    fields:|      - name:|        type:' | sed 's/^  //g' | while read line; do
  if [[ $line == *"- name:"* ]]; then
    TABLE_NAME=$(echo $line | awk '{print $3}')
    echo "Creating table: $TABLE_NAME"
    echo "DROP TABLE IF EXISTS $TABLE_NAME;" >> $SCHEMA_FILE
    echo "CREATE TABLE $TABLE_NAME (" >> $SCHEMA_FILE
  elif [[ $line == *"engine:"* ]]; then
    ENGINE=$(echo $line | awk '{print $2}')
  elif [[ $line == *"primary_key:"* ]]; then
    PRIMARY_KEY=$(echo $line | awk '{$1=""; print $0}' | sed 's/^[ \t]*//')
  elif [[ $line == *"fields:"* ]]; then
    continue
  elif [[ $line == *"- name:"* ]]; then
    FIELD_NAME=$(echo $line | awk '{print $3}')
    FIELD_NAMES+=($FIELD_NAME)
  elif [[ $line == *"type:"* ]]; then
    FIELD_TYPE=$(echo $line | awk '{print $2}')
    echo "  $FIELD_NAME $FIELD_TYPE," >> $SCHEMA_FILE
  elif [[ $line == *"maps:"* ]]; then
    # End of schema section
    echo ") ENGINE = $ENGINE" >> $SCHEMA_FILE
    if [[ -n "$PRIMARY_KEY" ]]; then
      echo "PRIMARY KEY ($PRIMARY_KEY)" >> $SCHEMA_FILE
    fi
    echo ";" >> $SCHEMA_FILE
    break
  fi
done

# Clean up the SQL file to fix any syntax issues
sed -i 's/,[ \t]*)/)/g' $SCHEMA_FILE

# Apply the schema
echo "Applying schema to ClickHouse..."
clickhouse-client $CLICKHOUSE_CONN --multiquery < $SCHEMA_FILE

echo "ClickHouse setup completed successfully!"
