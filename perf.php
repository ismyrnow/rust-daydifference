<?php

require_once 'DayDifference.php';

use johncorrelli\DayDifference;

// Script to call the function 2000 times with different dates
$start_dates = [];
$end_dates = [];
$n = 2000;
$start_day_offset = 6000;
$weekDays = [1, 2, 3, 4, 5];
$today = new DateTime();
$holidays = [];

// Generate random start and end dates
for ($i = 0; $i < $n; ++$i) {
    $start_date = new DateTime();
    $start_date->modify('-'.$start_day_offset + $i.' days');

    $end_date = clone $start_date;
    $end_date->modify('+'.($i % 30).' days');

    $start_dates[] = $start_date;
    $end_dates[] = $end_date;
}

$total_duration = 0;

// Measure execution time
$start_time = microtime(true);

// Call the function N times
for ($i = 0; $i < $n; ++$i) {
    $days = new DayDifference($today, $end_dates[$i], $weekDays, $holidays);
    $result = $days->difference();
}

$end_time = microtime(true);
$total_duration = $end_time - $start_time;

echo 'Total execution time for 2000 iterations: '.$total_duration." seconds\n";
