# Dilbert :: Design

**D**ilbert **I**s **L**ike **B**ats with **E**nhanced **R**ust **T**echnology


**(This project is in its infancy and should not be used by anyone)**


BATS is the Bash Automated Testing System. You can usefully think of it as a unit-testing framework for Bash scripts. In reality, it runs commands and makes assertions on its output and side-effects etc. So in a way, it's useful for integration-testing and monitoring for deployed systems. I used it to great effect with a complex system involving Kafka, Logstash instances, Elasticsearch, Rsyslog, and various other bespoke services, and it really helped to identify where problems exist quickly in a stack, particularly when there are multiple components in a distributed system.

BATS is useful, but:

* It only knows Pass and Fail states. Warning would be useful for purposes of monitoring
* It doesn’t export metrics (eg. for Prometheus or others). I had to make my own wrapper for BATS which made assumptions about the format of test names etc. That was exported by node-manager to be picked up by Prometheus and put on a Grafana dashboard.
* It doesn’t integrate with existing monitoring systems. Some systems (eg. Microsoft Systems Operations Management, or Zabbix) author tests on the management service. I want to make it easy to have the tests in the same place as the code that would be deployed to any environment. That way, you can have vendors provide tests for a system, and have that more easily integrated with different customer monitoring systems.
* I’m always concerned about reliability and correctness. Particularly if I’m making assertions that contain user-controlled input, such as malformed character encodings, etc.
* Some of the syntax is a bit obtuse and gets in the way. Although what the BATS developers achieved is quite frankly amazing given its a Bash shell script.
* To install BATS you end up cloning multiple git repositories, so installation can be a bit more annoying where it is not already available as an easily installed package.

Dilbert aspires to be a tool that can:

* **I want a testing/monitoring tool that can run commands and make assertions about its output etc.**
Basic tests should be baked in without recourse to running commands (eg. is port listening, is systemd service running, is process found, etc.)
* Stdout (and stderr) should be able to be easily accessed (json, yaml) or parsed (regexp)
* I can start being productive very quickly, without having to learn much new syntax and quirks. YAML would be preferable, sure it has issues, but it’s widely known in the DevOps space.
* It should be able to use something like the Common Expression Language to form policy rules as to whether a test has failed.
* Run tests locally; ideally the tests would be located on the local machine and deployed alongside the application deployment so it's easier to have tests in all environments that can reach deeply into different parts of the system.
* Test selection should be able to be filtered (eg. fast tests every 5 minutes, slow tests every hour)
* Tests can be run safely in a schedule, without concern for things like cron overlap etc.
* Tests should be able to be run in parallel, ideally with some dependency mechanism.
* Failing tests could potentially trigger actions… (eg. command, webhook)
* Metrics should be trivially available and exported in standard/common formats, particularly OpenTelemetry / Prometheus.
* **Errors and Warnings should be easy to integrate with other monitoring systems**
* **Decouple application-health logic from monitoring systems**
* Be able to give a glimpse of system health easily, such as in in /etc/motd
* **Tests must not cause potential harm to the system, so should be suitably resource constrained (eg. cgroups, limits, temporary directories, etc.)**

I also want a good project to learn Rust, and this seems like a good project to do so.



