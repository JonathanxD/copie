with import <nixpkgs>{};

rustPlatform.buildRustPackage rec {
  pname = "copie";
  version = "bleeding";

  src = builtins.filterSource (path: type: type != "directory" || (baseNameOf path != "target" && baseNameOf path != ".git")) ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with lib; {
    description = "copy with environment variables";
    homepage = "https://github.com/JonathanxD/copie";
    license = licenses.mit;
    maintainers = with maintainers; [
      {
        name = "Jonathan H. R. Lopes";
        github = "JonathanxD";
        githubId = 5360060;
        matrix = "@jonathanhrl:matrix.org";
        email = "joniweb01@gmail.com";
        keys = [{
            longkeyid = "rsa3072/0x4DF5FC43FD4FE9CC";
            fingerprint = "B2C2 7303 C091 E72F C62A  AB69 4DF5 FC43 FD4F E9CC";
        }];
      }
    ];
  };
}
