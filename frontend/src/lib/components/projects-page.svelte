<script lang="ts">
  // The landing page, ported from Projects.vue.
  import type { HomeProps } from '#lib/types.ts';

  let { props }: { props: HomeProps } = $props();
</script>

<p>{props.name}</p>
