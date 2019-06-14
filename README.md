# canister 🛢️ <a href="https://www.iqlusion.io"><img src="https://storage.googleapis.com/iqlusion-production-web/img/logo/iqlusion-rings-sm.png" alt="iqlusion" width="32" height="32"></a>

Deploy self-contained binaries from [GCP Container Registry] (gcr.io) as systemd service units.

Supported release artifact formats are tarballs and [Docker `scratch`]-based images (i.e. single-layer only) containing the compiled binaries, as fetched from a Docker registry (in the form of a tarball). Our (as in [@iqlusioninc]'s) internal use of this tool is primarily with a Docker scratch-based workflow using a Docker-based build system (namely [GCP Cloud Build]).

[GCP Container Registry]: https://cloud.google.com/container-registry/
[Docker `scratch`]: https://hub.docker.com/_/scratch/
[@iqlusioninc]: https://github.com/iqlusioninc
[GCP Cloud Build]: https://cloud.google.com/cloud-build/

## Status 

Under active development. Currently **alpha** quality. 

## License

Copyright © 2018 iqlusion

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
