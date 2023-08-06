# Rust-Socket-Sandbox

Rust-Socket-Sandbox is an actively developed tool that provides an intuitive interface for interacting with WebSocket servers. As a developer frequently working with real-time applications using WebSockets, I designed this tool to suit my personal needs and daily tasks. However, it's designed to be flexible, and if you find it useful in your own projects, you're welcome to use it!

## Core Features

- **Multiple User Simulation**: Simulate numerous users interacting with your WebSocket server. This feature allows you to understand how your server handles multiple concurrent connections.

- **Manual Message Control**: Send custom messages to the server, offering a way to thoroughly test how your server reacts to specific payloads.

## Future Plans

My main focus is on enhancing the Rust-Socket-Sandbox to provide even more functionality:

- **Stress Testing**: I'm working on adding stress testing capabilities, helping ensure that WebSocket servers can handle high-load scenarios.

- **Fuzzy Testing**: Fuzzy testing is in the works, which will allow for testing how a server handles unexpected or malformed inputs.

In addition to the features mentioned above, I'm looking to leverage the power and speed of Rust to simulate more users than would be feasible with other languages. By utilizing Rust's async capabilities and Tokio, along with a high-performance UI powered by Egui, I aim to create a tool that delivers a superior user experience while maintaining optimal performance.

## Platform Support

While I primarily use Windows for client-side development, I develop my servers on Linux, as any self-respecting server developer would do! Rust-Socket-Sandbox should work on both Windows and Linux, as well as MacOS. If you're using it on these platforms and encounter any issues, don't hesitate to let me know or open a PR. Your feedback would be much appreciated!

## License

Rust-Socket-Sandbox is licensed under the MIT. For more information, please see the LICENSE file.

## Note

This project is tailored to my personal requirements and workflow, and while I don't anticipate broad usage, you're welcome to use it if it suits your needs. There are no guarantees or support, but any feedback, suggestions, or contributions are appreciated. Enjoy using Rust-Socket-Sandbox!
