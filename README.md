# spark_submit_helper
A simple program, which submit the solution of subject, to test spark program at the Spark Competition in Taiwan.

* Pre-install
  ```
  curl https://sh.rustup.rs -sSf | sh
  ```
  
* Init Script
  ```
  # The script is used to help make needed directories and clone Code-Fight
  $ spark_submit_helper/bin/init.sh
  ```

* Install environment script
  ```
  # The script is used to install sbt
  $ spark_submit_helper/bin/install_env.sh
  ```

* Build Code
  ```
  $ cd spark_submit_helper/src
  $ cargo build
  ```
  
* How to use
  ```
  # Server Side
  $ cp spark_submit_helper/target/debug/spark_submit_helper ~/
  $ ./spark_submit_helper
  
  
  # Client Side
  $ curl -XPOST -d '{"user":"larry", "language":"python", "subject":"word_count", "solution":"def answer(data):\n    result = data.flatMap(lambda x: x.split(\" \")).map(lambda x: (x, 1)).reduceByKey(lambda x, y: x + y).sortBy(lambda x : x[0]).sortBy(lambda x: x[1],  ascending=False).collect()\n    return result"}'  localhost:3000/submit
  ```
