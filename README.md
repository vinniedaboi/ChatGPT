# ChatGPT Application

This project was created to help learn the basics of Rust, including calls and syntax.

## Overview

The application uses two main crates, `reqwest` and `egui`, to interact with the OpenAI API. It makes calls to the models, responding to user prompts directly from the application.

## Features

- **OpenAI API Integration**: Connects with the OpenAI API to leverage ChatGPT models.
- **User Interface**: Utilizes `egui` to create a simple and intuitive user interface for interacting with the ChatGPT models.

## Dependencies

- [reqwest](https://crates.io/crates/reqwest): A high-level HTTP library for making requests.
- [egui](https://crates.io/crates/egui): A simple, immediate mode GUI library for creating user interfaces.

## Getting Started

### Prerequisites

- Rust: Ensure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).

### Installation

1. **Clone the Repository**:
    ```sh
    git clone https://github.com/vinniedaboi/ChatGPT
    cd chatgpt-application
    ```

2. **Add Your OpenAI API Key**:
    - Create a `.env` file in the project root.
    - Add your OpenAI API key to the `.env` file:
        ```env
        OPENAI_API_KEY=your_api_key_here
        ```

3. **Build and Run the Application**:
    ```sh
    cargo build
    cargo run
    ```

## Usage

Once the application is running, you can enter prompts into the interface and receive responses from the ChatGPT models via the OpenAI API.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request or open an issue to discuss what you would like to change.

