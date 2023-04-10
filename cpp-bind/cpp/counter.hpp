#include <iostream>
#include <string>

class Counter {
public:
  Counter() { this->count = 0; }
  ~Counter() { std::cout << "Counter dtor called!\n"; }
  void Inc() {
    std::cout << "count: " << this->count << '\n';
    count++;
  }
  std::string GetText() { return "Count: " + std::to_string(this->count); }

private:
  int count;
};
