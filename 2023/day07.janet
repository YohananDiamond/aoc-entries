(def valid-cards-str "AKQJT98765432")

(def input-peg
  (do
    (defn make-hand [cards bid]
      {:cards cards :bid bid})

    (peg/compile
      ~{:main (any :hand)
        :hand (/ (* :s* (<- (repeat 5 (set ,valid-cards-str))) :s+ :number)
                 ,make-hand)
        :number (/ (<- :d+) ,scan-number)
        }
      )))

(defn find-all-index [needle haystack]
  (def arr @[])
  (loop [[i x] :pairs haystack
         :when (= x needle)]
    (array/push arr i))
  arr)

(defn find-index-any-n
  "Find n of any card, and return their indexes"
  [haystack n]

  (label -result
    (loop [needle :in valid-cards-str
           :let [ids (find-all-index needle haystack)
                 len (length ids)]
           :when (= n len)]
      (return -result ids))
    nil
    ))

(defn has-unique-configuration [hand counts]
  (def cards (map |$ (in hand :cards)))
  (label -result
    (loop [c :in counts]
      # find the c-pair
      (def ids (find-index-any-n cards c))

      # if we didn't find it, return false
      (when (nil? ids)
        (return -result false))

      # remove said cards from the deck so the rest can be analyzed
      (each i ids
        (set (cards i) nil))
      )

    # if all's good, return true
    true
    ))

(defn is-five-of-a-kind [hand]
  (has-unique-configuration hand [5]))

(defn is-four-of-a-kind [hand]
  (has-unique-configuration hand [4 1]))

(defn is-full-house [hand]
  (has-unique-configuration hand [3 2]))

(defn is-three-of-a-kind [hand]
  (has-unique-configuration hand [3 1 1]))

(defn is-two-pair [hand]
  (has-unique-configuration hand [2 2 1]))

(defn is-one-pair [hand]
  (has-unique-configuration hand [2 1 1 1]))

(defn is-high-card [hand]
  (has-unique-configuration hand [1 1 1 1 1]))

(defn get-basic-rank
  "Note: the basic rank IS NOT the rank mentioned in the problem description. This is a preliminary step."
  [h]
  (cond
    (is-five-of-a-kind h) 0
    (is-four-of-a-kind h) -1
    (is-full-house h) -2
    (is-three-of-a-kind h) -3
    (is-two-pair h) -4
    (is-one-pair h) -5
    (is-high-card h) -6
    true (error "Unreachable code")))

(defn has-lower-rank
  "Returns whether h1 has a lower rank than h2."
  [h1 h2]

  (def r1 (get-basic-rank h1))
  (def r2 (get-basic-rank h2))
  (cond
    (< r1 r2) true
    (> r1 r2) false
    true (label -result
           (loop [:let [{:cards c1} h1
                        {:cards c2} h2]
                  i :range [0 5]
                  :let [card-1 (in c1 i)
                        card-n-1 (find-index |(= card-1 $) valid-cards-str)
                        card-2 (in c2 i)
                        card-n-2 (find-index |(= card-2 $) valid-cards-str)]
                  ]
             (cond
               # in this case, lower is better, so we have to invert the conditions

               (> card-n-1 card-n-2)
               (return -result true)

               (< card-n-1 card-n-2)
               (return -result false))
             )

           # if we reached this, the two cards are identical, so let's... just return a false, I guess. I dunno.
           false)
    )
  )

(defn main [& args]
  (as-> (:read stdin :all) x
        (peg/match input-peg x)
        (sort x has-lower-rank)
        (reduce (fn [acc [i hand]]
                  (let [rank (inc i)
                        {:bid bid} hand]
                    (+ acc (* rank bid))))
                0 (pairs x))
        (pp x)
        )
  )
