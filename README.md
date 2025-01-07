# Test `Reqwest` Client vs `Hyper` client Speed

Compare the speed of reqwest client to a hyper client.

Some quick learnings:

- ~connection for reqwest seems quite a bit slower than Hyper's connection speed~
  - reqwest uses native-tls unless the feature gate is disabled with default-features = false
- first connection to a host is is slow compared second (when not reusing the client). Under the hood I - understand that the socket is being reused, so this is somewhat expected.
- resolving localhost rather than 127.0.0.1 can be expensive
- using the client for the first time is slow (both reqwest/hyper)
- reusing the client gives ~4-24x speedup over no reused
