(ns day11
  (:require [clojure.test :refer :all]))

; -> []stone
(defn divide [stone]
  ; (println "divide" stone (type stone))
  (let [mid (quot (count (str stone)) 2)
        [a b](split-at mid (str stone))]
      [(Integer/parseInt (apply str a)) (Integer/parseInt (apply str b))]))

; -> []stone
(defn spl [stone]
  ; (println "spl" stone)
  (if (= stone 0)
    [1]
    (if (even? (count (str stone)))
     (divide stone)
     [(* stone 2024)])))
  
; -> count
(defn soln [stones blinked-times target-blinks]
  ; (println stones blinked-times)
  (if (>= blinked-times target-blinks)
    (do (println "stones before counting" stones) (count stones))
    (soln (flatten (map (fn [stone] (spl stone)) stones)) (+ blinked-times 1) target-blinks))) 

(deftest test-divide
  (is (= (divide 123) [1 23])))

(deftest test-spl
  (is (= (spl 125) [253000]))
  (is (= (spl 17) [1 7])))

(deftest test-soln
  (is (= (soln [125 17] 0 1) 3))
  (is (= (soln [125 17] 0 2) 4))
  (is (= (soln [125 17] 0 6) 22)))

(run-tests)
