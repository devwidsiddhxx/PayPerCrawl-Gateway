pub fn is_ai_crawler(user_agent: &str) -> bool {
    let bots = [
        "GPTBot",
        "ClaudeBot",
        "PerplexityBot",
        "Google-Extended",
        "Bytespider",
        "CCBot",
    ];

    // Case-insensitive containment check
    let ua_lower = user_agent.to_lowercase();
    bots.iter().any(|bot| ua_lower.contains(&bot.to_lowercase()))
}
