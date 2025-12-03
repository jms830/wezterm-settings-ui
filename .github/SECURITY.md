# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in WezTerm Settings UI, please report it responsibly:

1. **Do not** create a public GitHub issue for security vulnerabilities
2. Send an email to the maintainers (check repository settings for contact info)
3. Provide detailed information about the vulnerability

We will respond to security reports within 48 hours and work with you to resolve the issue.

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Best Practices

When using WezTerm Settings UI:

- Never commit `.env` files or credentials
- Review generated Lua config before using in production
- Keep dependencies updated (`cargo update`)
- Run `cargo audit` to check for known vulnerabilities
