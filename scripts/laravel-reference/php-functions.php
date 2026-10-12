<?php

// What PHP answers for the functions the port depends on, as JSON
// (record.py writes it to tests/fixtures/golden/php-functions.json):
// round($x, 1) for the security status, and the year Carbon reads from
// a birthday (parsed in UTC, the app timezone).

$securityStatus = [
    -0.45, 0.45, -0.05, 0.05, 0.15, 0.25, 0.35, -0.35, 1.05, 2.25, -2.25, 4.95, -4.95,
    4.9999999, -9.999999, 5.0, -10.0, 0.0, -0.0, 0.04999999, -0.04999999,
    3.1415926535, -1.2345678901234, 0.1 + 0.2, 1.4499999999999999, 1.45, -1.45,
    0.000001, -7.123456789, 2.5500000000000003, 2.55, -2.55,
];

$rounded = array_map(
    fn (float $value): array => ['input' => $value, 'output' => round($value, 1)],
    $securityStatus,
);

echo json_encode(
    ['round_1' => $rounded],
    JSON_PRETTY_PRINT | JSON_PRESERVE_ZERO_FRACTION,
), "\n";
