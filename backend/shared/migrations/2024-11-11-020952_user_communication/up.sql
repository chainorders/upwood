/* plpgsql-language-server:disable validation */
   CREATE TABLE news_articles (
          id uuid PRIMARY KEY,
          title VARCHAR NOT NULL,
          label VARCHAR NOT NULL,
          content TEXT NOT NULL,
          image_url TEXT NOT NULL,
          article_url TEXT NOT NULL,
          created_at TIMESTAMP NOT NULL,
          order_index INTEGER NOT NULL
          );

   CREATE TABLE platform_updates (
          id uuid PRIMARY KEY,
          title VARCHAR NOT NULL,
          label VARCHAR NOT NULL,
          created_at TIMESTAMP NOT NULL,
          order_index INTEGER NOT NULL
          );

   CREATE TABLE maintenance_messages (
          id uuid PRIMARY KEY,
          message TEXT NOT NULL,
          created_at TIMESTAMP NOT NULL,
          order_index INTEGER NOT NULL
          );

   CREATE TABLE guides (
          id uuid PRIMARY KEY,
          title VARCHAR NOT NULL,
          label VARCHAR NOT NULL,
          created_at TIMESTAMP NOT NULL,
          order_index INTEGER NOT NULL
          );

   CREATE TABLE support_questions (
          id uuid PRIMARY KEY,
          cognito_user_id VARCHAR NOT NULL REFERENCES users (cognito_user_id) ON DELETE cascade,
          user_email VARCHAR NOT NULL,
          message TEXT NOT NULL,
          created_at TIMESTAMP NOT NULL
          );
