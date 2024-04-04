-- pg_stat_activity 学习资料：https://help.aliyun.com/zh/analyticdb-for-postgresql/user-guide/pg-stat-activity-view
-- pg_terminate_backend 学习资料：https://developer.aliyun.com/article/43408

-- 下面这段 sql，主要使用 pg_stat_activity 视图找到用户名为 app_user 或者 数据库名为 app_db 的 session 并中断它
SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE
 usename = 'app_user' OR datname = 'app_db';

-- 删库
DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

-- DEV ONLY - Dev only password (for local dev and unit test).
CREATE USER app_user PASSWORD 'dev_only_pwd';
CREATE DATABASE app_db owner app_user ENCODING = 'UTF-8';