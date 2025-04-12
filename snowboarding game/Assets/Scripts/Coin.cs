using UnityEngine;

public class Coin : MonoBehaviour
{
    [SerializeField] private int coinValue = 5;

    private void OnTriggerEnter2D(Collider2D other)
    {
        if (other.CompareTag("Player"))
        {
            FindFirstObjectByType<ScoreManager>().AddPoints(coinValue);
            Destroy(gameObject);
        }
    }
}
