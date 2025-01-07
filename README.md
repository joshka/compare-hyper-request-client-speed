# Test Reqwest Speed

Compare the speed of reqwest client to a hyper client.

Some quick learnings:

- connection for reqwest seems quite a bit slower than Hyper's connection speed
- first connection to a host is is slow compared second (when not reusing the client). Under the hood I - understand that the socket is being reused, so this is somewhat expected.
- resolving localhost rather than 127.0.0.1 can be expensive
- using the client for the first time is slow (both reqwest/hyper)
- reusing the client gives ~4-24x speedup over no reused
