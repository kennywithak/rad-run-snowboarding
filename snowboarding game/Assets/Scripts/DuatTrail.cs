using UnityEngine;

public class DuatTrail : MonoBehaviour
{
    [SerializeField] ParticleSystem dustParticles;

   

    void OnCollisionEnter2D(Collision2D other) 
    {
        if(other.gameObject.tag == "Ground")
        {
            dustParticles.Play();
        }
    }

    void OnCollisionExit2D(Collision2D other) 
    {
        if(other.gameObject.tag == "Ground")
        {
            dustParticles.Stop();
        }
    }
}