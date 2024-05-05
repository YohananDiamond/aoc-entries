(def input-peg
  (do
    (peg/compile
      ~{:main (any :line)
        :line (/ (* (any (* :number (any " "))) "\n") ,tuple)
        :number (/ (<- (* (at-most 1 "-") :d+)) ,scan-number)
        }
      )))

(defn diff
  "Returns an array where each index `i`'s value is the difference between `arr[i]` and its next element."
  [arr]

  (seq [:let [len (length arr)]
        i :range [1 len]
        :let [lhs (in arr (dec i))
              rhs (in arr i)]]
    (- rhs lhs)
    )
  )

(defn diffs-till-zero
  "Continuously differentiate `arr` until the result is an array of zeros. Returns the original `arr` and all subsequent differentiations."
  [arr]

  (def diffs @[arr])
  (var current arr)
  (while (-> (all |(= $ 0) current) (not))
    (def d (diff current))
    (array/push diffs d)
    (set current d)
    )
  diffs
  )

(defn extrapolate
  [arr]

  (def diffs (diffs-till-zero arr))
  (var val 0)
  (loop [:let [len (length diffs)]
         # still not fully sure how :down-to works lol
         i :down-to [(dec len) 0]]
    (+= val (last (in diffs i)))
    )
  val
  )

(defn main [& args]
  (as-> (:read stdin :all) x
        (peg/match input-peg x)
        (map extrapolate x)
        (sum x)
        (pp x)
        )
  )
