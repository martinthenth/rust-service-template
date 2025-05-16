#!/bin/bash

curl -i -X POST \
  -H "Accept:application/json" \
  -H "Content-Type:application/json" \
  localhost:8083/connectors/ \
  -d '{
    "name": "users-connector",
    "config": {
      "connector.class": "io.debezium.connector.postgresql.PostgresConnector",
      "tasks.max": "1",
      "database.hostname": "postgres",
      "database.port": "5432",
      "database.user": "postgres",
      "database.password": "postgres",
      "database.dbname": "users_dev",
      "slot.name": "users_connector_slot",
      "plugin.name": "pgoutput",
      "topic.prefix": "users_dev",
      "table.include.list" : "public.outbox",
      "transforms": "outbox",
      "transforms.outbox.type": "io.debezium.transforms.outbox.EventRouter",
      "transforms.outbox.route.by.field": "domain",
      "transforms.outbox.table.field.event.id": "id",
      "transforms.outbox.table.field.event.key": "key",
      "transforms.outbox.table.field.event.payload": "payload",
      "transforms.outbox.route.topic.replacement": "${routedByValue}",
      "transforms.outbox.table.fields.additional.placement": "type:header"
    }
  }'
