;; A small Emily program — the last form is the program's result.

(def x 42)

(def y
  (let [z 10]
    (if false z x)))

{:answer x :value y :tags [1 2 3]}
