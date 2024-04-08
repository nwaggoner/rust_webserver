# RUST Web Example
Nathan Waggoner 2024

This repo is for development and testing of RUST for TOP: Rust Web Development 005/105 Spring 2024.


/*  Goes in README.md
 *  Network Stuff
 * PHY      - packets on wire: header w/ address, payload
 *            (eth mac 0a:1b:2c:3d:4e:5f)
 * IPv4     - packets, headers, payload; wrapped in PHY
 *            (IPv4 addr 1.217.3.4)
 *            "best effort" aka it tries it's best to deliver information
 * TCP      - bidi streams, ports, split and wrapped in IPv4
 *            (16-bit port 3000)
 * HTPP     - text packets, headers and body, wrapped by TCP
 *            (textual URL http://1.217.3.4/)
 * HTML     - "special" text in HTTP body
 * TLS      - encryption for TCP streams, protects HTTP
 *            (used for "https")
 * DNS      - map names to IPv4 addresses
 *            (example.org -> 1.217.3.4)
 *
 */
