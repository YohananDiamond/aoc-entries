(def input-peg
  (peg/compile
    ~{:main (any :row)
      :row (/ (* :conditions-a :s+ :conditions-b "\n") ,tuple)
      :conditions-a (/ (some (<- (set ".#?"))) ,tuple)
      :conditions-b (/ (* (any (* :number ",")) :number) ,tuple)
      :number (/ (<- :d+) ,scan-number)
      }))

(def cond-count-peg
  (peg/compile
    ~{:main (any (+ "." :broken))
      :broken (/ (<- (some "#")) ,length)
      }))

(defn is-valid-arrangement [row]
  (def [ca cb] row)
  (def ca-str (string/join ca))
  (= (tuple ;(peg/match cond-count-peg ca-str)) cb))

(defn n-possible-arrangements [row]
  # I do wonder if there is an almost O(1) answer to this. But I don't have the big brain for that.
  # ... I think I should study combinatorics again.

  (def [ca cb] row)

  (defn count-valid-cases [c]
    (label result
           # recursion - when there's still a "?"
           (def i (find-index |(= $ "?") c))
           (unless (nil? i)
             (def is-last-element (= (inc i) (length c)))
             (def before (tuple/slice c 0 i))
             (def after (if is-last-element
                          [] (tuple/slice c (inc i))))
             (def possibilities
               [(tuple ;before "#" ;after)
                (tuple ;before "." ;after)])
             (as-> possibilities .v
                   (map count-valid-cases .v)
                   (sum .v)
                   (return result .v)
                   )
             )

           # base case - when there are no "?"
           (def valid (is-valid-arrangement [c cb]))
           (return result (if valid 1 0))
           ))

  (def result (count-valid-cases ca))
  (pp ~(result: ,result))
  result)

(defn main [& args]
  (as-> (:read stdin :all) .v
        (peg/match input-peg .v)
        (map n-possible-arrangements .v)
        (sum .v)
        (pp .v)
        )
  )
