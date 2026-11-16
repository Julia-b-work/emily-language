; Typed config, take one. Annotations parse, and the evaluator ignores them
; for now: nothing here is checked yet. The type checker arrives next.

(def server : Server
  {:host "localhost" :port 8080 :debug false})

(def make-config : (String -> Int -> Config)
  (fn (name : String port : Int)
    {:name name :port port :url (concat "http://" name ":" (show port))}))

{:server server
 :api (make-config "api" 8080)}
