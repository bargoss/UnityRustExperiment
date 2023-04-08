using Bubbles;
using UnityEngine;

namespace DefaultNamespace
{
    public class ArenaGameSinglePlayer : MonoBehaviour
    {
        ArenaFightGameExt m_Game;
        private int m_Tick;
        private float m_DeltaTime = 0.02f;

        void Start()
        {
            m_Game = Interop.create_game();
        }

        void FixedUpdate()
        {
            Interop.advance_tick(m_Game);
            Interop.register_views(m_Game);

            var time = m_Tick * m_DeltaTime;

            Interop.render(m_Game, time - m_DeltaTime * 0.5f, sphereInfo =>
            {
                var position = new Vector3(sphereInfo.position.x, sphereInfo.position.y, sphereInfo.position.z);
                var radius = sphereInfo.radius;
                var color = new Color(sphereInfo.color.x, sphereInfo.color.y, sphereInfo.color.z);
                DrawCircle(position, radius, color, 3);
            });

            m_Tick++;
        }

        void DrawCircle(Vector3 center, float radius, Color color, int points)
        {
            float time = Time.time;
            float rotationSpeed = 1.0f;
            float angleOffset = Mathf.Atan2(center.y, center.x) + rotationSpeed * time;
            var angle = angleOffset;
            angle += Mathf.PerlinNoise(center.x * 0.1f, center.y * 0.1f) * 90.5f;
            var angleIncrease = (2 * Mathf.PI) / points;
            var prev = Vector3.zero;
            for (var i = 0; i < points + 1; i++)
            {
                var x = center.x + (radius * Mathf.Cos(angle));
                var y = center.y + (radius * Mathf.Sin(angle));
                var pos = new Vector3(x, y, center.z);
                if (i > 0)
                {
                    Debug.DrawLine(prev, pos, color);
                }

                prev = pos;
                angle += angleIncrease;
            }
        }
    }



    /*
    void DrawCircle(Vector3 center, float radius, Color color, int points = 6) {
        var angle = 0f;
        var angleIncrease = (2 * Mathf.PI) / points;
        var prev = Vector3.zero;
        for (var i = 0; i < points + 1; i++) {
            var x = center.x + (radius * Mathf.Cos(angle));
            var y = center.y + (radius * Mathf.Sin(angle));
            var pos = new Vector3(x, y, center.z);
            if (i > 0) {
                Debug.DrawLine(prev, pos, color);
            }
            prev = pos;
            angle += angleIncrease;
        }
    }
    */
}