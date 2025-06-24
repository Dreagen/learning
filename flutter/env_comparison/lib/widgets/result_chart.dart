import 'package:env_comparison/utils/colors.dart';
import 'package:fl_chart/fl_chart.dart';
import 'package:flex_color_scheme/flex_color_scheme.dart';
import 'package:flutter/material.dart';

typedef ChartData = ({
  Map<String, int> adds,
  Map<String, int> mods,
  Map<String, int> dels,
});

class ResultChart extends StatelessWidget {
  final ChartData? chartData;
  ResultChart({super.key, this.chartData});

  final Color dark = Colors.cyan.darken(60);
  final Color normal = Colors.cyan.darken(30);
  final Color light = Colors.cyan;

  Widget bottomTitles(double value, TitleMeta meta) {
    const style = TextStyle(fontSize: 10);
    String text;
    switch (value.toInt()) {
      case 0:
        text = 'Adds';
      case 1:
        text = 'Mods';
      case 2:
        text = 'Dels';
      default:
        text = 'Default';
    }
    return SideTitleWidget(
      meta: meta,
      child: Text(text, style: style),
    );
  }

  Widget leftTitles(double value, TitleMeta meta) {
    if (value == meta.max) {
      return Container();
    }
    const style = TextStyle(fontSize: 10);
    return SideTitleWidget(
      meta: meta,
      child: Text(meta.formattedValue, style: style),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Positioned(
          top: 0,
          right: 8,
          child: Column(
            children: [
              Row(
                children: [
                  Container(width: 10, height: 10, color: AppColors.shareClass),
                  SizedBox(width: 5),
                  Text('Share class', style: TextStyle(fontSize: 12)),
                ],
              ),
              SizedBox(height: 8),
              Row(
                children: [
                  Container(width: 10, height: 10, color: AppColors.topic),
                  SizedBox(width: 5),
                  Text('Topic', style: TextStyle(fontSize: 12)),
                ],
              ),
              SizedBox(height: 8),
              Row(
                children: [
                  Container(
                    width: 10,
                    height: 10,
                    color: AppColors.mediaOutlet,
                  ),
                  SizedBox(width: 5),
                  Text('Media outlet', style: TextStyle(fontSize: 12)),
                ],
              ),
            ],
          ),
        ),
        AspectRatio(
          aspectRatio: 2,
          child: Padding(
            padding: const EdgeInsets.only(top: 16, right: 16),
            child: LayoutBuilder(
              builder: (context, constraints) {
                final barsSpace = 4.0 * constraints.maxWidth / 200;
                final barsWidth = 16.0 * constraints.maxWidth / 300;
                return BarChart(
                  BarChartData(
                    alignment: BarChartAlignment.center,
                    barTouchData: const BarTouchData(enabled: false),
                    titlesData: FlTitlesData(
                      show: true,
                      bottomTitles: AxisTitles(
                        sideTitles: SideTitles(
                          showTitles: true,
                          reservedSize: 28,
                          getTitlesWidget: bottomTitles,
                        ),
                      ),
                      leftTitles: AxisTitles(
                        sideTitles: SideTitles(
                          showTitles: false,
                          reservedSize: 40,
                          getTitlesWidget: leftTitles,
                        ),
                      ),
                      topTitles: const AxisTitles(
                        sideTitles: SideTitles(showTitles: false),
                      ),
                      rightTitles: const AxisTitles(
                        sideTitles: SideTitles(showTitles: false),
                      ),
                    ),
                    gridData: FlGridData(
                      show: true,
                      checkToShowHorizontalLine: (value) => value % 10 == 0,
                      getDrawingHorizontalLine: (value) => FlLine(
                        color: Colors.red.withValues(alpha: 0.1),
                        strokeWidth: 1,
                      ),
                      drawVerticalLine: false,
                    ),
                    borderData: FlBorderData(show: false),
                    // groupsSpace: barsSpace,
                    groupsSpace: 10,
                    barGroups: getData(barsWidth, barsSpace),
                  ),
                );
              },
            ),
          ),
        ),
      ],
    );
  }

  List<BarChartGroupData> getData(double barsWidth, double barsSpace) {
    var addsCurrent = 0.0;
    var modsCurrent = 0.0;
    var delsCurrent = 0.0;
    return [
      BarChartGroupData(
        x: 0,
        barsSpace: barsSpace,
        showingTooltipIndicators: [],
        barRods: [
          BarChartRodData(
            toY: chartData!.adds.values.fold(0, (sum, elem) => sum + elem),
            rodStackItems: chartData!.adds.entries.map((x) {
              var rodStackItem = BarChartRodStackItem(
                addsCurrent,
                addsCurrent + x.value,
                getColorForDataType(x.key),
              );
              addsCurrent += x.value;

              return rodStackItem;
            }).toList(),
            borderRadius: BorderRadius.zero,
            width: barsWidth,
          ),
        ],
      ),
      BarChartGroupData(
        x: 1,
        barsSpace: barsSpace,
        showingTooltipIndicators: [],
        barRods: [
          BarChartRodData(
            toY: chartData!.mods.values.fold(0, (sum, elem) => sum + elem),
            rodStackItems: chartData!.mods.entries.map((x) {
              var rodStackItem = BarChartRodStackItem(
                modsCurrent,
                modsCurrent + x.value,
                getColorForDataType(x.key),
              );

              modsCurrent += x.value;
              return rodStackItem;
            }).toList(),
            borderRadius: BorderRadius.zero,
            width: barsWidth,
          ),
        ],
      ),
      BarChartGroupData(
        x: 2,
        barsSpace: barsSpace,
        showingTooltipIndicators: [],
        barRods: [
          BarChartRodData(
            toY: chartData!.dels.values.fold(0, (sum, elem) => sum + elem),
            rodStackItems: chartData!.dels.entries.map((x) {
              var rodStackItem = BarChartRodStackItem(
                delsCurrent,
                delsCurrent + x.value,
                getColorForDataType(x.key),
              );

              delsCurrent += x.value;
              return rodStackItem;
            }).toList(),
            borderRadius: BorderRadius.zero,
            width: barsWidth,
          ),
        ],
      ),
    ];
  }

  Color getColorForDataType(String key) {
    return switch (key.toLowerCase()) {
      'topic' => AppColors.topic,
      'shareclass' => AppColors.shareClass,
      'mediaoutletmap' => AppColors.mediaOutlet,
      _ => Colors.green,
    };
  }
}
