<script lang="ts">
  export let videoId: String;
</script>

<style>
  /* Estilo opcional para ajustar el tamaño del video */
  iframe {
    width: 100%;
    height: 400px; /* Puedes ajustar la altura según tus necesidades */
  }
</style>

<div>
  <iframe
    src={`https://www.youtube.com/embed/${videoId}`}
    frameborder="0"
    allowfullscreen
    title="video"
  ></iframe>
</div>