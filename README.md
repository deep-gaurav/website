# Personal Portfolio Website

This repository contains my personal portfolio website as a software engineer. It's built with Leptos, a Rust web framework, using Server-Side Generation (SSG) and Tailwind CSS for styling. The website features a custom Content Management System (CMS) for the projects page, utilizing Markdown files.

**Live Website:** [https://deepgaurav.com/](https://deepgaurav.com/)

## Features

- Built with Leptos Rust framework
- Server-Side Generation (SSG) for improved performance
- Tailwind CSS for responsive and customizable styling
- Custom CMS for projects page using Markdown files
- Showcase of my software engineering projects and skills

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos)
- wasm32-unknown-unknown target

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/deep-gaurav/website.git
   cd website
   ```

2. Install cargo-leptos:
   ```
   cargo install cargo-leptos
   ```

3. Add the wasm32-unknown-unknown target:
   ```
   rustup target add wasm32-unknown-unknown
   ```

## Running the Project

To run the project in release mode and generate static files:

```
cargo leptos serve --release
```

This command will:
1. Compile the Rust code
2. Generate all static files in the `target/site` directory
3. Serve the website using a built-in file server


## Deployment

To deploy the website, you can use any static file hosting service. Simply upload the contents of the `target/site` directory to your hosting provider.

The live version of this portfolio is currently hosted at [https://deepgaurav.com/](https://deepgaurav.com/).

## Project Showcase

The projects page uses a custom CMS based on Markdown files. To add or edit projects:

1. Navigate to the `projects` directory
2. Create a new Markdown file or edit an existing one
3. Follow the established format for project entries using frontmatter

Each Markdown file in this directory represents a project in your portfolio. Metadata is provided using frontmatter at the beginning of each file.

### Project Metadata Structure

| Frontmatter Key | Required | Description |
|-----------------|----------|-------------|
| title | Yes | The title of your project |
| cover | Yes | Path or URL to the project's cover image |
| tagline | Yes | A brief, catchy phrase describing the project |
| short_description | Yes | A concise summary of the project (1-2 sentences) |
| stack | No | Technologies used in the project |
| web_url | No | URL to the live project (if applicable) |
| play_store_url | No | URL to the Play Store listing (for Android apps) |
| backend_source | No | URL to the backend source code repository |
| frontend_source | No | URL to the frontend source code repository |


## Customization

Feel free to customize the website to reflect your personal brand and showcase your unique skills and experiences. You can modify the Rust components, update the Tailwind CSS styles, or adjust the content structure as needed.

## Contributing

As this is a personal portfolio website, contributions are not expected. However, if you notice any issues or have suggestions, feel free to open an issue in the repository.

## Acknowledgements

Site design inspired by [purplefolio](https://dribbble.com/shots/23212629-Purplefolio-Framer-Portfolio-Website-For-Web-Developers) by [Luca Da Corte](https://dribbble.com/LucaDaCorte)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
