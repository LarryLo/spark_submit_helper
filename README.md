# spark_submit_helper
A simple program, which submit the solution of subject, to test the Spark Competition in Taiwan.

* Pre-install
  * Rust ~1.17.0
  
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
  $ cp spark_submit_helper/target/debug/spark_submit_helper ~/
  $ spark_submit_helper -l python -s word_count
  ```
