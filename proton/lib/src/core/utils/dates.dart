
import 'mixins/enums.dart' show EnumMixin;

enum Day with EnumMixin implements Comparable<Day> {
  monday,
  tuesday,
  wednesday,
  thursday,
  friday,
  saturday,
  sunday;

  static Day fromIndex(int index) {
    return Day.values[index];
  }

  @override
  int compareTo(Day other) {
    return index.compareTo(other.index);
  }
}

enum Period implements Comparable<Period> {
  am,
  pm;

  static Period fromIndex(int index) {
    return Period.values[index];
  }

  static Period fromDate(DateTime date) {
    return date.hour < 12 ? Period.am : Period.pm;
  }

  @override
  int compareTo(Period other) {
    return index.compareTo(other.index);
  }

  String get label => name.toUpperCase();
}

enum Month with EnumMixin implements Comparable<Month> {
  january,
  february,
  march,
  april,
  may,
  june,
  july,
  august,
  september,
  october,
  november,
  december;

  static Month fromIndex(int index) {
    return Month.values[index];
  }

  static Month fromDate(DateTime date) {
    return Month.values[date.month - 1];
  }

  @override
  int compareTo(Month other) {
    return index.compareTo(other.index);
  }
}

extension DateTimeExt on DateTime {
  String get displayDate {
    return '$day ${monthEnum.label} $year';
  }

  String get displayTime {
    int hour = this.hour % 12;
    int minute = this.minute;
    String period = this.period.label;
    if (minute < 10) {
      return '$hour:0$minute $period';
    }
    return '$hour:$minute $period';
  }

  Period get period => Period.fromDate(this);

  Month get monthEnum => Month.fromDate(this);
}
