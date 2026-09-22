;; Build every endpoint URL from one base — change one line, update all.

(def api {:scheme "https" :host "api.example.com" :version "v1"})

(def url (fn (path)
  (concat api.scheme "://" api.host "/" api.version path)))

{:users (url "/users")
 :orders (url "/orders")
 :invoices (url "/invoices")}
