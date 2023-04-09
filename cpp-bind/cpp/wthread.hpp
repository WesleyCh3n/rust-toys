#include <iostream>

#include <atomic>
#include <mutex>
#include <thread>

class WThread {
public:
  WThread() {
    this->thread_ = nullptr;
    this->stop_ = false;
  };
  ~WThread() {
    std::cout << "WThread dtor called\n";
    this->Stop();
  };

  void Start() {
    if (!thread_) {
      thread_ = new std::thread(&WThread::Run, this);
      this->stop_ = false;
    }
  };
  void Stop() {
    if (thread_) {
      std::cout << "Stop called\n";
      this->stop_ = true;

      this->thread_->join();
      this->thread_ = nullptr;
    }
  };

private:
  void Run() {
    while (!this->stop_) {
      std::cout << "Thread Running\n";
      std::this_thread::sleep_for(std::chrono::milliseconds(500));
    }
  };
  std::thread *thread_;
  std::atomic<bool> stop_;
};
