<script>
  import { Editor, Viewer } from 'bytemd'
  import gfm from '@bytemd/plugin-gfm'
  import 'bytemd/dist/index.css';
  
  let plugins = [gfm()]

  let value = '';

</script>


<Editor class="prose-xl prose-slate max-w-none" {plugins} {value} on:change={(e) => {value = e.detail.value}}/>
