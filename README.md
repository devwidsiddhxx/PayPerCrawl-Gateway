# PayPerCrawl Gateway

An edge proxy written in Rust that monetizes AI crawler traffic using cryptographic micropayments.

## Overview

This project is a high-performance reverse proxy built with Rust, Axum, and Tokio. It inspects incoming HTTP requests for known AI crawler User-Agents (like `GPTBot`, `ClaudeBot`, etc.). 
- If a normal user visits the site, the request is transparently proxied to your origin server.
- If an AI crawler is detected, the gateway checks for a valid Ed25519-signed JSON token in the `X-Payment-Proof` header.
- Valid tokens allow the crawler to pass; invalid or missing tokens result in an `HTTP 402 Payment Required`.

This provides a proof-of-concept for how website owners can require AI companies to pay per page crawled.

## Architecture

```text
Browser / AI Bot
       │
       ▼
 ┌──────────────┐      Valid      ┌──────────────┐
 │ Rust Gateway │ ──────────────▶ │ Origin Server│
 └──────────────┘                 └──────────────┘
       │
       ▼ Invalid / Missing Proof
 HTTP 402 Payment Required
```

## Running Locally

The easiest way to run the gateway safely on your local machine is using Docker Compose. This starts:
1. The **PayPerCrawl Gateway** on `http://localhost:8080`
2. A **Demo Origin Website** (internal only)
3. **Prometheus** for metrics on `http://localhost:9090`
4. **Grafana** for metrics visualization on `http://localhost:3000`

```bash
docker-compose up --build
```

## Demo Scenario

1. **Human Traffic (Allowed)**
   ```bash
   curl http://localhost:8080
   # Returns: 200 OK - Welcome to Demo Blog
   ```

2. **AI Crawler without Payment (Blocked)**
   ```bash
   curl -A "GPTBot" http://localhost:8080
   # Returns: 402 Payment Required
   ```

3. **Generate a Mock Payment Proof**
   Since this is a demo, you can ask the gateway to mint a valid proof for you:
   ```bash
   curl http://localhost:8080/mock-payment
   # Returns a JSON with a valid token
   ```

4. **AI Crawler with Payment (Allowed)**
   ```bash
   curl -A "GPTBot" -H "X-Payment-Proof: <YOUR_TOKEN>" http://localhost:8080
   # Returns: 200 OK - Welcome to Demo Blog
   ```

## Metrics

Visit `http://localhost:8080/metrics` to see the raw Prometheus metrics. You will see:
- `requests_passed_total`
- `requests_blocked_total`
- `payment_revenue_total`

## Security Notes

- This runs purely locally. It will not intercept traffic on your network or modify your browser.
- The default target origin is the demo website on port `8081`. No real scraping occurs.
- Cryptographic keys used in `config.rs` are hardcoded for demo purposes only. Do not use them in production!
