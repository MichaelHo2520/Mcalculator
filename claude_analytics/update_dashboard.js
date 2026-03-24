const fs = require('fs');
const path = require('path');
const os = require('os');

const PROJECTS_DIR = path.join(os.homedir(), '.claude', 'projects');
const OUTPUT_FILE = path.join(__dirname, 'dashboard_data.json');

// Constants for estimation (Pro Plan limits)
const LIMIT_5H = 45; // Estimated messages per 5 hours
const LIMIT_7D = 500; // Estimated total for 7 days (placeholder)

function parseLogs() {
    const stats = {
        totalTokens: { input: 0, output: 0, cacheRead: 0, cacheCreate: 0 },
        sessions: [],
        allMessages: [],
        lastUpdate: new Date().toISOString()
    };

    if (!fs.existsSync(PROJECTS_DIR)) {
        console.error('Projects directory not found:', PROJECTS_DIR);
        return stats;
    }

    const projectFolders = fs.readdirSync(PROJECTS_DIR);

    projectFolders.forEach(folder => {
        const folderPath = path.join(PROJECTS_DIR, folder);
        if (fs.lstatSync(folderPath).isDirectory()) {
            const files = fs.readdirSync(folderPath).filter(f => f.endsWith('.jsonl'));
            files.forEach(file => {
                const filePath = path.join(folderPath, file);
                const content = fs.readFileSync(filePath, 'utf8');
                const lines = content.split('\n').filter(l => l.trim());

                lines.forEach(line => {
                    try {
                        const data = JSON.parse(line);
                        if (data.type === 'user' || data.type === 'assistant' || (data.message && data.message.usage)) {
                            const timestamp = new Date(data.timestamp || Date.now());
                            const usage = (data.message && data.message.usage) || data.usage;

                            if (usage) {
                                stats.totalTokens.input += usage.input_tokens || 0;
                                stats.totalTokens.output += usage.output_tokens || 0;
                                stats.totalTokens.cacheRead += usage.cache_read_input_tokens || usage.cacheReadTokens || 0;
                                stats.totalTokens.cacheCreate += usage.cache_creation_input_tokens || 0;
                            }

                            if (data.type === 'user' || (data.message && data.message.role === 'user')) {
                                stats.allMessages.push({
                                    timestamp: timestamp.getTime(),
                                    type: 'user'
                                });
                            }
                        }
                    } catch (e) {
                        // Skip malformed lines
                    }
                });
            });
        }
    });

    // Sort messages by time
    stats.allMessages.sort((a, b) => a.timestamp - b.timestamp);

    // Calculate Reset Timers
    const now = Date.now();
    const window5H = 5 * 60 * 60 * 1000;
    const window7D = 7 * 24 * 60 * 60 * 1000;

    const messagesLast5H = stats.allMessages.filter(m => m.timestamp > now - window5H);
    const messagesLast7D = stats.allMessages.filter(m => m.timestamp > now - window7D);

    stats.usage5H = {
        count: messagesLast5H.length,
        limit: LIMIT_5H,
        percent: Math.round((messagesLast5H.length / LIMIT_5H) * 100),
        nextReset: messagesLast5H.length > 0 ? messagesLast5H[0].timestamp + window5H : null
    };

    stats.usage7D = {
        count: messagesLast7D.length,
        limit: LIMIT_7D,
        percent: Math.round((messagesLast7D.length / LIMIT_7D) * 100),
        nextReset: messagesLast7D.length > 0 ? messagesLast7D[0].timestamp + window7D : null
    };

    // Current Session (messages since last gap > 1 hour)
    let sessionStartIdx = 0;
    for (let i = stats.allMessages.length - 1; i > 0; i--) {
        if (stats.allMessages[i].timestamp - stats.allMessages[i-1].timestamp > 60 * 60 * 1000) {
            sessionStartIdx = i;
            break;
        }
    }
    stats.currentSession = {
        count: stats.allMessages.length - sessionStartIdx,
        startTime: stats.allMessages.length > 0 ? stats.allMessages[sessionStartIdx].timestamp : null
    };

    fs.writeFileSync(OUTPUT_FILE, JSON.stringify(stats, null, 2));
    console.log('Dashboard data updated at:', stats.lastUpdate);
}

parseLogs();
