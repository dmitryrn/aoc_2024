#include <algorithm>
#include <fstream>
#include <iostream>
#include <iterator>
#include <string>
#include <unordered_map>
#include <vector>
#include <array>

using namespace std;

void part1(array<int, 1000> list1, array<int, 1000> list2)
{
  sort(begin(list1), end(list1));
  sort(begin(list2), end(list2));
  int distance_sum = 0;
  for (size_t i = 0; i < 1000; i++) {
    int distance = abs(list1[i] - list2[i]);
    distance_sum += distance;
  }
  cout << distance_sum << endl;
}

void part2(array<int, 1000> list1, array<int, 1000> list2)
{
  unordered_map<int, int> counters;

  for (int n : list2) {
    counters[n] += 1;
  }

  int similarity_score = 0;
  for (int n : list1) {
    similarity_score += n * counters[n];
  }

  cout << similarity_score << endl;
}

int main() {
  ifstream f("day1.txt");
  if (!f.is_open()) {
    cerr << "error opening file";
    return 1;
  }

  array<int, 1000> list1;
  array<int, 1000> list2;
  string s;
  int i = 0;
  while (getline(f, s)) {
    size_t idx = s.find(' ');

    string first_str = s.substr(0, idx);
    int first = stoi(first_str);

    string second_str = s.substr(idx + 3, -1);
    int second = stoi(second_str);

    list1[i] = first;
    list2[i] = second;

    i++;
  }

  part1(list1, list2); // vectors are passed by value here
  part2(list1, list2);

  f.close();
}
