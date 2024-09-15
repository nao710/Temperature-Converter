#include <iostream>

float toFahrenheit(float celsius) {
  float fahrenheit = celsius * 1.8 + 32;
  return fahrenheit;
}

float toCelsius(float fahrenheit) {
  float celsius = (fahrenheit - 32) * 5 / 9;
  return celsius;
}

int main() {
  std::cout << "どちらを変換しますか？" << std::endl;
  std::cout << "(1) 摂氏⇒ 華氏" << std::endl;
  std::cout << "(2) 華氏⇒ 摂氏" << std::endl;

  int num;
  std::cin >> num;

  switch (num) {
    case 1: {
      float celsius;
      std::cout << "摂氏を入力" << std::endl;
      std::cin >> celsius;
      float fahrenheit = toFahrenheit(celsius);
      std::cout << "華氏" << fahrenheit << "℉ です";
    } break;
    case 2: {
      float fahrenheit;
      std::cout << "華氏を入力" << std::endl;
      std::cin >> fahrenheit;
      float celsius = toCelsius(fahrenheit);
      std::cout << "摂氏" << celsius << "℃ です";
    } break;
    default:
      std::cout << "1か2を入力してください";
  }

  return 0;
}
