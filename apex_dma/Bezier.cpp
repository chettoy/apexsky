#include "Bezier.h"

namespace BezierNS {
    QAngle Bezier(const std::vector<QAngle>& points, float t) {
        if (points.size() == 1) {
            return points[0];
        }

        std::vector<QAngle> new_points;
        for (size_t i = 0; i < points.size() - 1; i++) {
            QAngle p = points[i] + (points[i + 1] - points[i]) * t;
            new_points.push_back(p);
        }

        return Bezier(new_points, t);
    }
}