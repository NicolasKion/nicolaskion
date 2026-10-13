<script lang="ts">
  // The page has no shared chrome; the layout only pulls in the styles.
  import '../app.css';

  let { children } = $props();
</script>

{@render children()}
