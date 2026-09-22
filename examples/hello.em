;; A configuration, computed with code!

(def base {:host "localhost" :port 8080 :debug false})

(def prod (merge base {:host "prod.example.com"}))

(def services ["api" "Emily" "scheduler"])

(def instances
  (fold services [] (fn (acc name)
    (append acc {:name name
                 :url (concat "http://" prod.host ":" (show prod.port))}))))

{:env "production" :config prod :instances instances}
