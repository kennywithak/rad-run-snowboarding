using UnityEngine;

public class PlayerController : MonoBehaviour
{
    [Header("Movement")]
    [SerializeField] float torqueAmount = 3f;      // Adjust these in Inspector
    [SerializeField] float boostSpeed = 50f;
    [SerializeField] float baseSpeed = 30f;

    [Header("Scoring")]
    [SerializeField] private ScoreManager scoreManager;
    [SerializeField] private float rotationThreshold = 0.5f;

    private Rigidbody2D rb2d;
    private SurfaceEffector2D surfaceEffector2D;
    private bool canMove = true;
    private float totalRotation = 0f;
    private float lastFrameRotation;
    private bool hasStarted = false;

    void Start()
    {
        rb2d = GetComponent<Rigidbody2D>();
        surfaceEffector2D = FindFirstObjectByType<SurfaceEffector2D>();

        if (scoreManager == null)
        {
            scoreManager = FindFirstObjectByType<ScoreManager>();
        }

        lastFrameRotation = rb2d.rotation;

        // Optional: Force 60 FPS in WebGL
        Application.targetFrameRate = 60;
    }

    void Update()
    {
        if (canMove)
        {
            if (!hasStarted && (Input.GetKey(KeyCode.LeftArrow) ||
                                Input.GetKey(KeyCode.RightArrow) ||
                                Input.GetKey(KeyCode.UpArrow)))
            {
                hasStarted = true;
                scoreManager.StartTimer();
            }

            DetectTricks();
        }
    }

    void FixedUpdate()
    {
        if (canMove)
        {
            HandleRotation();
            HandleBoost();
        }
    }

    public void DisableControls()
    {
        canMove = false;
    }

    void HandleBoost()
    {
        if (Input.GetKey(KeyCode.UpArrow))
        {
            surfaceEffector2D.speed = boostSpeed;
        }
        else
        {
            surfaceEffector2D.speed = baseSpeed;
        }
    }

    void HandleRotation()
    {
        if (Input.GetKey(KeyCode.LeftArrow))
        {
            rb2d.AddTorque(torqueAmount);
        }
        else if (Input.GetKey(KeyCode.RightArrow))
        {
            rb2d.AddTorque(-torqueAmount);
        }
    }

    void DetectTricks()
    {
        float currentRotation = rb2d.rotation;
        float rotationDelta = currentRotation - lastFrameRotation;

        if (rotationDelta > 180f) rotationDelta -= 360f;
        if (rotationDelta < -180f) rotationDelta += 360f;

        totalRotation += Mathf.Abs(rotationDelta);

        if (totalRotation >= 360f)
        {
            scoreManager.AddPoints(10);
            totalRotation -= 360f;
        }

        lastFrameRotation = currentRotation;
    }
}
