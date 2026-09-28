; Split-config example: import two files, then combine them into one config.
(import "database.em")
(import "server.em")

{:name "my-app"
 :db db
 :server server
 :connection (concat "postgres://" db.host ":" (show db.port) "/" db.name)}
