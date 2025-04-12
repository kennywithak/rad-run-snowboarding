using UnityEngine;
using TMPro;
using System.Collections;

public class BlinkText : MonoBehaviour
{
    [SerializeField] private TextMeshProUGUI textObject;
    [SerializeField] private float blinkRate = 0.5f;

    private void Start()
    {
        StartCoroutine(Blink());
    }

    IEnumerator Blink()
    {
        while (true)
        {
            textObject.enabled = !textObject.enabled;
            yield return new WaitForSeconds(blinkRate);
        }
    }
}

