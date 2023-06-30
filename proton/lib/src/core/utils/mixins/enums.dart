import '../strings.dart';

mixin EnumMixin on Enum {
  String get label => name.capitalize();
}
