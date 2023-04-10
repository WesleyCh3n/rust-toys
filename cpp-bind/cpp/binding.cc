#include "counter.hpp"
#include "wthread.hpp"

extern "C" {
Counter *counter_factory() { return new Counter(); }
void counter_destructor(Counter *counter) { delete counter; }
void counter_inc(Counter *counter) { counter->Inc(); };
}

extern "C" {
WThread *wthread_ctor() { return new WThread(); }
void wthread_dtor(WThread *ptr) { delete ptr; }
void wthread_start(WThread *ptr) { ptr->Start(); }
void wthread_stop(WThread *ptr) { ptr->Stop(); }
}

extern "C" {
float *get_vec() {
  auto *vec = new float(10);
  for (int i = 0; i < 10; i++) {
    vec[i] = (float)i;
  }
  return vec;
}
void free_vec(float *ptr) { delete[] ptr; }
}
