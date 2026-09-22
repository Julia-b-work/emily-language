;; One base config, three environments — no copy-pasting.

(def base {:host "localhost" :port 8080 :debug false})

(def staging (merge base {:host "staging.example.com" :debug true}))

(def prod (merge base {:host "prod.example.com"}))

{:dev base :staging staging :prod prod}
