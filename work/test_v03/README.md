nohup ./target/debug/test_v03 &
ps -ef | grep test
curl http://localhost:3000/
seq 1 100 | parallel -j10 curl -o /dev/null -s -w "%{time_total}\n" http://localhost:3000/vendors/2650041
RUST_LOG=info cargo run