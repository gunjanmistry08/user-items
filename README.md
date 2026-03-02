# User Items

```bash
curl -X POST http://localhost:8080/users -H "Content-Type: application/json" -d '{"email": "gmax@x.com"}'


kafka-console-consumer.sh --topic test-topic --bootstrap-server localhost:9092 --from-beginning


kafka-console-producer --topic test-topic --bootstrap-server localhost:9092


kafka-topics --bootstrap-server localhost:9092 --describe --topic users
```
