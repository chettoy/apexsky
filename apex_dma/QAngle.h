#pragma once

#include <cmath>

class QAngle {
public:
    float x, y, z;

    QAngle() : x(0), y(0), z(0) {}
    QAngle(float x, float y, float z) : x(x), y(y), z(z) {}

    void Normalize() {
        while (x > 89) x -= 180;
        while (x < -89) x += 180;
        while (y > 180) y -= 360;
        while (y < -180) y += 360;
        z = 0;
    }

    friend QAngle operator+(const QAngle& lhs, const QAngle& rhs) {
        return QAngle(lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z);
    }

    friend QAngle operator-(const QAngle& lhs, const QAngle& rhs) {
        return QAngle(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z);
    }

    friend QAngle operator*(const QAngle& lhs, float rhs) {
        return QAngle(lhs.x * rhs, lhs.y * rhs, lhs.z * rhs);
    }

    friend QAngle operator/(const QAngle& lhs, float rhs) {
        return QAngle(lhs.x / rhs, lhs.y / rhs, lhs.z / rhs);
    }

    friend bool operator==(const QAngle& lhs, const QAngle& rhs) {
        return lhs.x == rhs.x && lhs.y == rhs.y && lhs.z == rhs.z;
    }

    friend bool operator!=(const QAngle& lhs, const QAngle& rhs) {
        return !(lhs == rhs);
    }
};