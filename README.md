# HTTP Server From Scratch

A lightweight HTTP server built from the ground up in **Rust**, without relying on web frameworks like Axum or Actix. The goal of this project is to understand what really happens under the hood of an HTTP server: how raw TCP connections are accepted, how requests are read and parsed, and how responses are constructed and sent back.

Once a request is parsed, the extracted data drives the rest of the server, including **error handling**, **routing**, and the **user-facing interface** that people see in their browser.
