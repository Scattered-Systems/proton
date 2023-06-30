import 'package:flutter/material.dart';

import '../strings.dart';

mixin TabEnumMixin on Enum {
  String get label => name.capitalize();

  Tab get textTab => Tab(text: label);
}
