CREATE TABLE users (
        id INTEGER PRIMARY KEY,
            -- username TEXT NOT NULL UNIQUE,
                email TEXT NOT NULL UNIQUE,
                    password TEXT NOT NULL,
                        subscription_plan TEXT NOT NULL,
                            data_usage INTEGER NOT NULL DEFAULT 0,

                                is_admin INTEGER NOT NULL DEFAULT 0,
                                    is_banned INTEGER NOT NULL DEFAULT 0,
                                        created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now')),
                                            updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%S', 'now'))
                                        );

