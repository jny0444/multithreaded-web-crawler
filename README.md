# Multithreaded Web Crawler

A rust multi threaded web crawler to download images and learn how concurrency works in rust.

## Benchmarks

|                               | no of images | time taken (in seconds) |
| ----------------------------- | ------------ | ----------------------- |
| Single Threaded               | 20           | 17.590181459            |
| Multi Threaded (Simple)       | 20           | 1.698569833             |
| Multi Threaded (MPMC channel) | 20           | 6.396878042             |

> Note: You see Simple multithreaded is giving better time than MPMC channel because its less number of images, once the number of images is high, there would be more threads spawned and might choke the device on which it is running and the MPMC with the same number of images would be much stable
